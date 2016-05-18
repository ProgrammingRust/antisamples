enum AminoAcid {
    Ala, Arg, Asn, Asp, Cys, Gln, Glu, Gly, His, Ile,
    Leu, Lys, Met, Phe, Pro, Ser, Thr, Trp, Tyr, Val
}

mod proteins {
    // proteins.rs
    pub fn synthesize(seq: &[AminoAcid])  // error: AminoAcid is undefined
    //~^ ERROR: type name `AminoAcid` is undefined
        -> Protein
    {
        Protein
    }

    pub struct Protein;
}

