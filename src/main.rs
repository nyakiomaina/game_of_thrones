struct Kingdom {
    name: String,
    ruler: String,
}

fn main() {
    let kingdoms = vec![
        Kingdom {
            name: "The North".to_string(),
            ruler: "House Stark".to_string(),
        },
        Kingdom {
            name: "The Vale".to_string(),
            ruler: "House Arryn".to_string(),
        },
        Kingdom {
            name: "The Riverlands".to_string(),
            ruler: "House Tully".to_string(),
        },
        Kingdom {
            name: "The Westerlands".to_string(),
            ruler: "House Lannister".to_string(),
        },
        Kingdom {
            name: "The Iron Islands".to_string(),
            ruler: "House Greyjoy".to_string(),
        },
        Kingdom {
            name: "The Reach".to_string(),
            ruler: "House Tyrell".to_string(),
        },
        Kingdom {
            name: "The Stormlands".to_string(),
            ruler: "House Baratheon".to_string(),
        },
        Kingdom {
            name: "Dorne".to_string(),
            ruler: "House Martell".to_string(),
        },
        Kingdom {
            name: "The Crownlands".to_string(),
            ruler: "House Baratheon of King's Landing".to_string(),
        },
    ];

    for kingdom in kingdoms {
        println!("The Kingdom of {}: Ruled by {}", kingdom.name, kingdom.ruler);
    }
}
