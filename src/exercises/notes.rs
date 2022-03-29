mod neck_note;
mod paper_note;

pub use neck_note::NeckNote;
pub use paper_note::PaperNote;

pub fn init_notes() -> Vec<(PaperNote, Vec<NeckNote>)> {
    let notes = vec![
        // E
        (PaperNote::D43, vec![NeckNote::N6_0]),
        // F
        (PaperNote::D3, vec![NeckNote::N6_1]),
        // A
        (PaperNote::D2, vec![NeckNote::N6_5, NeckNote::N5_0]),
        // B
        (PaperNote::D21, vec![NeckNote::N6_7, NeckNote::N5_2]),
        // C
        (PaperNote::D1, vec![NeckNote::N6_8, NeckNote::N5_3]),
        // D
        (
            PaperNote::M01,
            vec![NeckNote::N6_10, NeckNote::N5_5, NeckNote::N4_0],
        ),
        // E
        (
            PaperNote::M1,
            vec![NeckNote::N6_12, NeckNote::N5_7, NeckNote::N4_2],
        ),
        // F
        (
            PaperNote::M12,
            vec![NeckNote::N6_13, NeckNote::N5_8, NeckNote::N4_3],
        ),
        // G
        (
            PaperNote::M2,
            vec![
                NeckNote::N6_15,
                NeckNote::N5_10,
                NeckNote::N4_5,
                NeckNote::N3_0,
            ],
        ),
        // A
        (
            PaperNote::M23,
            vec![
                NeckNote::N6_17,
                NeckNote::N5_12,
                NeckNote::N4_7,
                NeckNote::N3_2,
            ],
        ),
        // B
        (
            PaperNote::M3,
            vec![
                NeckNote::N6_19,
                NeckNote::N5_14,
                NeckNote::N4_9,
                NeckNote::N3_4,
                NeckNote::N2_0,
            ],
        ),
        // C
        (
            PaperNote::M34,
            vec![
                NeckNote::N6_20,
                NeckNote::N5_15,
                NeckNote::N4_10,
                NeckNote::N3_5,
                NeckNote::N2_1,
            ],
        ),
        // D
        (
            PaperNote::M4,
            vec![
                NeckNote::N5_17,
                NeckNote::N4_12,
                NeckNote::N3_7,
                NeckNote::N2_3,
            ],
        ),
        // E
        (
            PaperNote::M45,
            vec![
                NeckNote::N5_19,
                NeckNote::N4_14,
                NeckNote::N3_9,
                NeckNote::N2_5,
                NeckNote::N1_0,
            ],
        ),
        // F
        (
            PaperNote::M5,
            vec![
                NeckNote::N5_20,
                NeckNote::N4_15,
                NeckNote::N3_10,
                NeckNote::N2_6,
                NeckNote::N1_1,
            ],
        ),
        // G
        (
            PaperNote::M56,
            vec![
                NeckNote::N4_17,
                NeckNote::N3_12,
                NeckNote::N2_8,
                NeckNote::N1_3,
            ],
        ),
        // A
        (
            PaperNote::U1,
            vec![
                NeckNote::N4_19,
                NeckNote::N3_14,
                NeckNote::N2_10,
                NeckNote::N1_5,
            ],
        ),
        // B
        (
            PaperNote::U12,
            vec![NeckNote::N3_16, NeckNote::N2_12, NeckNote::N1_7],
        ),
        // C
        (
            PaperNote::U2,
            vec![NeckNote::N3_17, NeckNote::N2_13, NeckNote::N1_8],
        ),
        // D
        (
            PaperNote::U23,
            vec![NeckNote::N3_19, NeckNote::N2_15, NeckNote::N1_10],
        ),
        // E
        (PaperNote::U3, vec![NeckNote::N2_17, NeckNote::N1_12]),
        // F
        (PaperNote::U34, vec![NeckNote::N2_18, NeckNote::N1_13]),
    ];
    notes
}
