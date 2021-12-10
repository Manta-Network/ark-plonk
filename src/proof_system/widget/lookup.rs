// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! Lookup Gates

/* TODO:
use crate::proof_system::GateConstraint;
use crate::proof_system::GateValues;
*/
use ark_ff::Field;
use core::marker::PhantomData;

/* TODO:
///
pub struct Lookup<F>
where
    F: Field,
{
    ///
    f_i: F,

    ///
    p_i: F,

    ///
    p_i_next: F,

    ///
    t_i: F,

    ///
    t_i_next: F,

    ///
    h_1_i: F,

    ///
    h_1_i_next: F,

    ///
    h_2_i: F,

    ///
    l_first_i: F,

    ///
    delta: F,

    ///
    epsilon: F,

    ///
    zeta: F,
}

impl<F> Lookup<F>
where
    F: Field,
{
    #[inline]
    fn constraints(separation_challenge: &F, values: GateValues<F>) -> F {
        (compress(
            values.left,
            values.right,
            values.output,
            values.fourth,
            values.lookup_compression,
        ) - values.lookup)
            * separation_challenge
    }

    /*
    #[inline]
    fn constraints(
        &self,
        separation_challenge: &F,
        values: GateValues<F>,
    ) -> F {
        let challenge_sq = separation_challenge.square();
        let challenge_cu = challenge_sq * separation_challenge;

        let one_plus_delta = delta + F::one();
        let epsilon_one_plus_delta = epsilon * one_plus_delta;

        // q_lookup(X) * (a(X) + zeta * b(X) + (zeta^2 * c(X)) + (zeta^3 * d(X)
        // - f(X))) * α_1
        let a = {
            let q_lookup_i = self.q_lookup.1[index];
            let compressed_tuple =
                compress(*w_l_i, *w_r_i, *w_o_i, *w_4_i, *zeta);
            q_lookup_i * (compressed_tuple - f_i) * separation_challenge
        };

        // L0(X) * (p(X) − 1) * α_1^2
        let b = { l_first_i * (p_i - F::one()) * challenge_sq };

        // p(X) * (1+δ) * (ε+f(X)) * (ε*(1+δ) + t(X) + δt(Xω)) * α_1^3
        let c = {
            let c_1 = epsilon + f_i;
            let c_2 = epsilon_one_plus_delta + t_i + delta * t_i_next;
            p_i * one_plus_delta * c_1 * c_2 * challenge_cu
        };

        // − p(Xω) * (ε*(1+δ) + h1(X) + δ*h2(X)) * (ε*(1+δ) + h2(X) + δ*h1(Xω))
        // * α_1^3
        let d = {
            let d_1 = epsilon_one_plus_delta + h_1_i + delta * h_2_i;
            let d_2 = epsilon_one_plus_delta + h_2_i + delta * h_1_i_next;
            -p_i_next * d_1 * d_2 * challenge_cu
        };

        a + b + c + d
    }
    */
}

#[inline]
fn compress(w_l: F, w_r: F, w_o: F, w_4: F, zeta: F) -> F {
    // TODO: See if Horner's method is faster here.
    let zeta_sq = zeta.square();
    let zeta_cu = zeta_sq * zeta;
    w_l + (w_r * zeta) + (w_o * zeta_sq) + (w_4 * zeta_cu)
}
*/
