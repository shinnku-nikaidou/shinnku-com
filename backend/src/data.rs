// Data module containing in-memory databases used by handlers
#[allow(dead_code)]
pub static INTRO_DB: &[(&str, &str)] = &[
    ("kant", "German philosopher known for his moral philosophy."),
    ("tesla", "Serbian-American inventor and engineer."),
    (
        "einstein",
        "Physicist who developed the theory of relativity.",
    ),
];

#[allow(dead_code)]
pub static FINDNAME_DB: &[(&str, &str)] = &[
    (
        "Immanuel Kant",
        "Prominent figure of the Enlightenment and author of the 'Critique of Pure Reason'.",
    ),
    (
        "Nikola Tesla",
        "Inventor who contributed to the design of the modern alternating current electricity supply system.",
    ),
    (
        "Albert Einstein",
        "Theoretical physicist famous for the theory of relativity and mass–energy equivalence formula E=mc^2.",
    ),
];
