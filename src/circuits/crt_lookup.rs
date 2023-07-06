use super::* ;

struct FLGateChip<F>(GateChip<F>);

impl<F: ScalarField> FLGateChip<F> {
    pub fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        ab: impl Into<QuantumCell<F>>,
         modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        let (a, b, ab) = into_crt(a, b, ab, modulus);
        a
    }

    pub fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        ab: impl Into<QuantumCell<F>>,
        modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        let (a, b, ab) = into_crt(a, b, ab, modulus);
        a
    }
}