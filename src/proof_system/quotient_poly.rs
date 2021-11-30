// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::proof_system::ecc::CurveAddition;
use crate::proof_system::ecc::FixedBaseScalarMul;
use crate::proof_system::logic::Logic;
use crate::proof_system::range::Range;
use crate::proof_system::GateConstraint;
use crate::proof_system::GateValues;
use crate::{error::Error, proof_system::ProverKey};
use ark_ec::TEModelParameters;
use ark_ff::PrimeField;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain,
    UVPolynomial,
};

/// Computes the Quotient [`DensePolynomial`] given the [`EvaluationDomain`], a
/// [`ProverKey`] and some other info.
pub(crate) fn compute<F: PrimeField, P: TEModelParameters<BaseField = F>>(
    domain: &GeneralEvaluationDomain<F>,
    prover_key: &ProverKey<F, P>,
    z_poly: &DensePolynomial<F>,
    w_l_poly: &DensePolynomial<F>,
    w_r_poly: &DensePolynomial<F>,
    w_o_poly: &DensePolynomial<F>,
    w_4_poly: &DensePolynomial<F>,
    public_inputs_poly: &DensePolynomial<F>,
    alpha: &F,
    beta: &F,
    gamma: &F,
    range_challenge: &F,
    logic_challenge: &F,
    fixed_base_challenge: &F,
    var_base_challenge: &F,
) -> Result<DensePolynomial<F>, Error> {
    // Compute `4n` eval of `z(X)`.
    let domain_4n =
        GeneralEvaluationDomain::<F>::new(4 * domain.size()).unwrap();
    let mut z_eval_4n = domain_4n.coset_fft(z_poly);
    z_eval_4n.push(z_eval_4n[0]);
    z_eval_4n.push(z_eval_4n[1]);
    z_eval_4n.push(z_eval_4n[2]);
    z_eval_4n.push(z_eval_4n[3]);

    // Compute `4n` evaluations of the wire dense polynomials.
    let mut wl_eval_4n = domain_4n.coset_fft(w_l_poly);
    wl_eval_4n.push(wl_eval_4n[0]);
    wl_eval_4n.push(wl_eval_4n[1]);
    wl_eval_4n.push(wl_eval_4n[2]);
    wl_eval_4n.push(wl_eval_4n[3]);
    let mut wr_eval_4n = domain_4n.coset_fft(w_r_poly);
    wr_eval_4n.push(wr_eval_4n[0]);
    wr_eval_4n.push(wr_eval_4n[1]);
    wr_eval_4n.push(wr_eval_4n[2]);
    wr_eval_4n.push(wr_eval_4n[3]);
    let wo_eval_4n = domain_4n.coset_fft(w_o_poly);

    let mut w4_eval_4n = domain_4n.coset_fft(w_4_poly);
    w4_eval_4n.push(w4_eval_4n[0]);
    w4_eval_4n.push(w4_eval_4n[1]);
    w4_eval_4n.push(w4_eval_4n[2]);
    w4_eval_4n.push(w4_eval_4n[3]);

    let t_1 = compute_circuit_satisfiability_equation(
        domain,
        *range_challenge,
        *logic_challenge,
        *fixed_base_challenge,
        *var_base_challenge,
        prover_key,
        &wl_eval_4n,
        &wr_eval_4n,
        &wo_eval_4n,
        &w4_eval_4n,
        public_inputs_poly,
    );

    let t_2 = compute_permutation_checks(
        domain,
        prover_key,
        (&wl_eval_4n, &wr_eval_4n, &wo_eval_4n, &w4_eval_4n),
        &z_eval_4n,
        (*alpha, *beta, *gamma),
    );
    let range = 0..domain_4n.size();

    let quotient = range
        .map(|i| {
            let numerator = t_1[i] + t_2[i];
            let denominator = prover_key.v_h_coset_4n()[i];
            numerator * denominator.inverse().unwrap()
        })
        .collect::<Vec<_>>();

    Ok(DensePolynomial {
        coeffs: domain_4n.coset_ifft(&quotient),
    })
}

/// Ensures that the circuit is satisfied.
fn compute_circuit_satisfiability_equation<F, P>(
    domain: &GeneralEvaluationDomain<F>,
    range_challenge: F,
    logic_challenge: F,
    fixed_base_challenge: F,
    var_base_challenge: F,
    prover_key: &ProverKey<F, P>,
    wl_eval_4n: &[F],
    wr_eval_4n: &[F],
    wo_eval_4n: &[F],
    w4_eval_4n: &[F],
    pi_poly: &DensePolynomial<F>,
) -> Vec<F>
where
    F: PrimeField,
    P: TEModelParameters<BaseField = F>,
{
    let domain_4n =
        GeneralEvaluationDomain::<F>::new(4 * domain.size()).unwrap();
    let pi_eval_4n = domain_4n.coset_fft(pi_poly);
    (0..domain_4n.size())
        .map(|i| {
            let pi = pi_eval_4n[i];

            let values = GateValues {
                left: wl_eval_4n[i],
                right: wr_eval_4n[i],
                output: wo_eval_4n[i],
                fourth: w4_eval_4n[i],
                left_next: wl_eval_4n[i + 4],
                right_next: wr_eval_4n[i + 4],
                fourth_next: w4_eval_4n[i + 4],
                left_selector: prover_key.left_selector.1[i],
                right_selector: prover_key.right_selector.1[i],
                output_selector: prover_key.output_selector.1[i],
                fourth_selector: prover_key.fourth_selector.1[i],
                constant_selector: prover_key.constant_selector.1[i],
            };

            let arithmetic = prover_key.arithmetic.quotient_term(i, values);

            let range = Range::quotient_term(
                prover_key.range_selector.1[i],
                range_challenge,
                values,
            );

            let logic = Logic::quotient_term(
                prover_key.logic_selector.1[i],
                logic_challenge,
                values,
            );

            let fixed_base_scalar_mul =
                FixedBaseScalarMul::<_, P>::quotient_term(
                    prover_key.fixed_group_add_selector.1[i],
                    fixed_base_challenge,
                    values,
                );

            let curve_addition = CurveAddition::<_, P>::quotient_term(
                prover_key.variable_group_add_selector.1[i],
                var_base_challenge,
                values,
            );

            (arithmetic + pi)
                + range
                + logic
                + fixed_base_scalar_mul
                + curve_addition
        })
        .collect()
}

///
fn compute_permutation_checks<
    F: PrimeField,
    P: TEModelParameters<BaseField = F>,
>(
    domain: &GeneralEvaluationDomain<F>,
    prover_key: &ProverKey<F, P>,
    (wl_eval_4n, wr_eval_4n, wo_eval_4n, w4_eval_4n): (&[F], &[F], &[F], &[F]),
    z_eval_4n: &[F],
    (alpha, beta, gamma): (F, F, F),
) -> Vec<F> {
    let domain_4n =
        GeneralEvaluationDomain::<F>::new(4 * domain.size()).unwrap();
    let l1_poly_alpha =
        compute_first_lagrange_poly_scaled(domain, alpha.square());
    let l1_alpha_sq_evals = domain_4n.coset_fft(&l1_poly_alpha.coeffs);

    let range = 0..domain_4n.size();

    let t: Vec<_> = range
        .map(|i| {
            prover_key.permutation.compute_quotient_i(
                i,
                wl_eval_4n[i],
                wr_eval_4n[i],
                wo_eval_4n[i],
                w4_eval_4n[i],
                z_eval_4n[i],
                z_eval_4n[i + 4],
                alpha,
                l1_alpha_sq_evals[i],
                beta,
                gamma,
            )
        })
        .collect();
    t
}

///
fn compute_first_lagrange_poly_scaled<F: PrimeField>(
    domain: &GeneralEvaluationDomain<F>,
    scale: F,
) -> DensePolynomial<F> {
    let mut x_evals = vec![F::zero(); domain.size()];
    x_evals[0] = scale;
    domain.ifft_in_place(&mut x_evals);
    DensePolynomial::from_coefficients_vec(x_evals)
}
