use std::collections::HashMap;

use informed_search_algorithms::{pretty_print_path, InformedSearch, Node, Strategies};

fn main() {
    let state_space_graph = HashMap::from([
        (
            "Addis Ababa",
            Node::new(
                "Addis Ababa",
                26,
                vec![
                    ("Adama", 3),
                    ("Ambo", 5),
                    ("Debre Berhan", 5),
                    ("Debre Markos", 13),
                ],
            ),
        ),
        (
            "Adama",
            Node::new(
                "Adama",
                23,
                vec![
                    ("Matahara", 3),
                    ("Asella", 4),
                    ("Batu", 4),
                    ("Addis Ababa", 3),
                ],
            ),
        ),
        (
            "Ambo",
            Node::new(
                "Ambo",
                31,
                vec![("Wolkite", 6), ("Addis Ababa", 5), ("Nekemte", 8)],
            ),
        ),
        (
            "Debre Berhan",
            Node::new(
                "Debre Berhan",
                31,
                vec![("Addis Ababa", 5), ("Debre Sina", 2)],
            ),
        ),
        (
            "Debre Markos",
            Node::new(
                "Debre Markos",
                39,
                vec![("Addis Ababa", 13), ("Debre Sina", 17), ("Finote Selam", 3)],
            ),
        ),
        (
            "Matahara",
            Node::new("Matahara", 26, vec![("Adama", 3), ("Awash", 1)]),
        ),
        (
            "Asella",
            Node::new("Asella", 22, vec![("Adama", 4), ("Assasa", 4)]),
        ),
        (
            "Batu",
            Node::new(
                "Batu",
                19,
                vec![("Adama", 4), ("Buta Jirra", 2), ("Shashamene", 3)],
            ),
        ),
        (
            "Wolkite",
            Node::new(
                "Wolkite",
                25,
                vec![
                    ("Ambo", 6),
                    ("Worabe", 5),
                    ("Jimma", 8),
                    ("Hossana", 5),
                    ("Buta Jirra", 4),
                ],
            ),
        ),
        (
            "Nekemte",
            Node::new(
                "Nekemte",
                39,
                vec![("Ambo", 8), ("Bedelle", 4), ("Gimbi", 4)],
            ),
        ),
        (
            "Debre Sina",
            Node::new(
                "Debre Sina",
                33,
                vec![("Debre Berhan", 2), ("Kemise", 7), ("Debre Markos", 17)],
            ),
        ),
        (
            "Finote Selam",
            Node::new(
                "Finote Selam",
                42,
                vec![("Debre Markos", 3), ("Bahirdar", 6), ("Injibara", 2)],
            ),
        ),
        (
            "Awash",
            Node::new(
                "Awash",
                27,
                vec![("Chiro", 4), ("Gobi Rasu", 5), ("Matahara", 1)],
            ),
        ),
        (
            "Assasa",
            Node::new("Assasa", 18, vec![("Asella", 4), ("Dodolla", 1)]),
        ),
        (
            "Buta Jirra",
            Node::new(
                "Buta Jirra",
                21,
                vec![("Batu", 2), ("Wolkite", 4), ("Worabe", 2)],
            ),
        ),
        (
            "Shashamene",
            Node::new(
                "Shashamene",
                16,
                vec![
                    ("Batu", 3),
                    ("Dodolla", 3),
                    ("Hawassa", 1),
                    ("Hossana", 7),
                    ("Worabe", 6),
                ],
            ),
        ),
        (
            "Worabe",
            Node::new(
                "Worabe",
                22,
                vec![
                    ("Wolkite", 5),
                    ("Hossana", 2),
                    ("Shashamene", 6),
                    ("Buta Jirra", 2),
                ],
            ),
        ),
        (
            "Jimma",
            Node::new(
                "Jimma",
                33,
                vec![("Wolkite", 8), ("Bonga", 4), ("Bedelle", 7)],
            ),
        ),
        (
            "Hossana",
            Node::new(
                "Hossana",
                21,
                vec![
                    ("Shashamene", 7),
                    ("Worabe", 2),
                    ("Wolkite", 5),
                    ("Wolaita Sodo", 4),
                ],
            ),
        ),
        (
            "Bedelle",
            Node::new(
                "Bedelle",
                40,
                vec![("Nekemte", 4), ("Gore", 6), ("Jimma", 7)],
            ),
        ),
        (
            "Gimbi",
            Node::new(
                "Gimbi",
                43,
                vec![("Nekemte", 4), ("Dambidollo", 6), ("Assosa", 8)],
            ),
        ),
        (
            "Kemise",
            Node::new("Kemise", 40, vec![("Debre Sina", 7), ("Dessie", 4)]),
        ),
        (
            "Bahirdar",
            Node::new(
                "Bahirdar",
                48,
                vec![
                    ("Finote Selam", 6),
                    ("Injibara", 4),
                    ("Metekel", 11),
                    ("Azezo", 7),
                    ("Debre Tabor", 4),
                ],
            ),
        ),
        (
            "Injibara",
            Node::new("Injibara", 44, vec![("Bahirdar", 4), ("Finote Selam", 2)]),
        ),
        (
            "Chiro",
            Node::new("Chiro", 31, vec![("Awash", 4), ("Dire Dawa", 8)]),
        ),
        (
            "Gobi Rasu",
            Node::new("Gobi Rasu", 32, vec![("Awash", 5), ("Samara", 10)]),
        ),
        (
            "Dodolla",
            Node::new(
                "Dodolla",
                19,
                vec![("Assasa", 1), ("Shashamene", 3), ("Robe", 13)],
            ),
        ),
        (
            "Hawassa",
            Node::new("Hawassa", 15, vec![("Shashamene", 1), ("Dilla", 3)]),
        ),
        (
            "Bonga",
            Node::new(
                "Bonga",
                33,
                vec![
                    ("Jimma", 4),
                    ("Dawro", 10),
                    ("Tepi", 8),
                    ("Mizan Teferi", 4),
                ],
            ),
        ),
        (
            "Wolaita Sodo",
            Node::new(
                "Wolaita Sodo",
                17,
                vec![("Arba Minchi", 4), ("Dawro", 6), ("Hossana", 4)],
            ),
        ),
        (
            "Gore",
            Node::new(
                "Gore",
                46,
                vec![("Tepi", 9), ("Gambella", 5), ("Bedelle", 6)],
            ),
        ),
        (
            "Dambidollo",
            Node::new(
                "Dambidollo",
                49,
                vec![("Gimbi", 6), ("Assosa", 12), ("Gambella", 4)],
            ),
        ),
        (
            "Assosa",
            Node::new("Assosa", 51, vec![("Gimbi", 8), ("Dambidollo", 12)]),
        ),
        (
            "Dessie",
            Node::new("Dessie", 44, vec![("Kemise", 4), ("Woldia", 6)]),
        ),
        ("Metekel", Node::new("Metekel", 59, vec![("Bahirdar", 11)])),
        (
            "Azezo",
            Node::new(
                "Azezo",
                55,
                vec![("Gondar", 1), ("Bahirdar", 7), ("Metema", 7)],
            ),
        ),
        (
            "Debre Tabor",
            Node::new(
                "Debre Tabor",
                52,
                vec![("Lalibella", 8), ("Gondar", 6), ("Bahirdar", 4)],
            ),
        ),
        (
            "Dire Dawa",
            Node::new("Dire Dawa", 31, vec![("Chiro", 8), ("Harar", 4)]),
        ),
        (
            "Samara",
            Node::new(
                "Samara",
                42,
                vec![
                    ("Gobi Rasu", 10),
                    ("Fanti Rasu", 7),
                    ("Alamata", 11),
                    ("Woldia", 8),
                ],
            ),
        ),
        (
            "Robe",
            Node::new(
                "Robe",
                22,
                vec![
                    ("Liben", 11),
                    ("Dodolla", 13),
                    ("Goba", 18),
                    ("Sof Oumer", 23),
                ],
            ),
        ),
        (
            "Dilla",
            Node::new("Dilla", 12, vec![("Hawassa", 3), ("Bulehora", 4)]),
        ),
        (
            "Dawro",
            Node::new("Dawro", 23, vec![("Bonga", 10), ("Wolaita Sodo", 6)]),
        ),
        (
            "Tepi",
            Node::new(
                "Tepi",
                41,
                vec![("Gore", 9), ("Bonga", 8), ("Mizan Teferi", 4)],
            ),
        ),
        (
            "Mizan Teferi",
            Node::new("Mizan Teferi", 37, vec![("Tepi", 4), ("Bonga", 4)]),
        ),
        (
            "Gambella",
            Node::new("Gambella", 51, vec![("Gore", 5), ("Dambidollo", 4)]),
        ),
        (
            "Arba Minchi",
            Node::new(
                "Arba Minchi",
                13,
                vec![("Wolaita Sodo", 4), ("Konso", 4), ("Basketo", 10)],
            ),
        ),
        (
            "Woldia",
            Node::new(
                "Woldia",
                50,
                vec![
                    ("Dessie", 6),
                    ("Lalibella", 7),
                    ("Samara", 8),
                    ("Alamata", 3),
                ],
            ),
        ),
        (
            "Gondar",
            Node::new(
                "Gondar",
                56,
                vec![
                    ("Azezo", 1),
                    ("Humera", 9),
                    ("Metema", 7),
                    ("Debarke", 4),
                    ("Debre Tabor", 6),
                ],
            ),
        ),
        (
            "Metema",
            Node::new(
                "Metema",
                62,
                vec![("Azezo", 7), ("Gondar", 7), ("Kartum", 19)],
            ),
        ),
        (
            "Lalibella",
            Node::new(
                "Lalibella",
                57,
                vec![("Woldia", 7), ("Debre Tabor", 8), ("Sekota", 6)],
            ),
        ),
        (
            "Harar",
            Node::new("Harar", 35, vec![("Dire Dawa", 4), ("Babile", 2)]),
        ),
        (
            "Fanti Rasu",
            Node::new("Fanti Rasu", 49, vec![("Samara", 7), ("Kilbet Rasu", 6)]),
        ),
        (
            "Alamata",
            Node::new(
                "Alamata",
                53,
                vec![("Samara", 11), ("Woldia", 3), ("Mekelle", 5), ("Sekota", 6)],
            ),
        ),
        (
            "Liben",
            Node::new("Liben", 11, vec![("Robe", 11), ("Moyale", 11)]),
        ),
        (
            "Goba",
            Node::new(
                "Goba",
                40,
                vec![("Robe", 18), ("Sof Oumer", 6), ("Babile", 28)],
            ),
        ),
        (
            "Sof Oumer",
            Node::new(
                "Sof Oumer",
                45,
                vec![("Goba", 6), ("Robe", 23), ("Gode", 23)],
            ),
        ),
        (
            "Bulehora",
            Node::new("Bulehora", 8, vec![("Dilla", 4), ("Yabello", 2)]),
        ),
        (
            "Konso",
            Node::new("Konso", 9, vec![("Arba Minchi", 4), ("Yabello", 3)]),
        ),
        (
            "Basketo",
            Node::new("Basketo", 23, vec![("Arba Minchi", 10), ("Bench Maji", 5)]),
        ),
        (
            "Humera",
            Node::new(
                "Humera",
                65,
                vec![("Shire", 8), ("Gondar", 9), ("Kartum", 21)],
            ),
        ),
        (
            "Debarke",
            Node::new("Debarke", 60, vec![("Gondar", 4), ("Shire", 7)]),
        ),
        (
            "Sekota",
            Node::new(
                "Sekota",
                59,
                vec![("Alamata", 6), ("Mekelle", 9), ("Lalibella", 6)],
            ),
        ),
        (
            "Babile",
            Node::new(
                "Babile",
                37,
                vec![("Harar", 2), ("Jigjiga", 3), ("Goba", 28)],
            ),
        ),
        (
            "Kilbet Rasu",
            Node::new("Kilbet Rasu", 55, vec![("Fanti Rasu", 6)]),
        ),
        (
            "Mekelle",
            Node::new(
                "Mekelle",
                58,
                vec![("Alamata", 5), ("Adigrat", 4), ("Adwa", 7), ("Sekota", 9)],
            ),
        ),
        (
            "Gode",
            Node::new(
                "Gode",
                35,
                vec![
                    ("Dollo", 17),
                    ("Kebri Dehar", 5),
                    ("Sof Oumer", 23),
                    ("Mogadishu", 22),
                ],
            ),
        ),
        (
            "Yabello",
            Node::new(
                "Yabello",
                6,
                vec![("Bulehora", 2), ("Konso", 3), ("Moyale", 6)],
            ),
        ),
        (
            "Bench Maji",
            Node::new("Bench Maji", 28, vec![("Basketo", 5), ("Juba", 22)]),
        ),
        (
            "Shire",
            Node::new(
                "Shire",
                67,
                vec![("Axum", 2), ("Humera", 8), ("Debarke", 7)],
            ),
        ),
        (
            "Jigjiga",
            Node::new("Jigjiga", 40, vec![("Babile", 3), ("Dega Habur", 5)]),
        ),
        (
            "Adigrat",
            Node::new(
                "Adigrat",
                62,
                vec![("Mekelle", 4), ("Adwa", 4), ("Asmera", 9)],
            ),
        ),
        (
            "Adwa",
            Node::new(
                "Adwa",
                65,
                vec![("Mekelle", 7), ("Axum", 1), ("Adigrat", 4)],
            ),
        ),
        (
            "Dollo",
            Node::new("Dollo", 18, vec![("Gode", 17), ("Moyale", 18)]),
        ),
        (
            "Kebri Dehar",
            Node::new(
                "Kebri Dehar",
                40,
                vec![("Gode", 5), ("Dega Habur", 6), ("Werder", 6)],
            ),
        ),
        (
            "Moyale",
            Node::new(
                "Moyale",
                0,
                vec![
                    ("Dollo", 18),
                    ("Liben", 11),
                    ("Yabello", 6),
                    ("Nairobi", 22),
                    ("Mogadishu", 40),
                ],
            ),
        ),
        (
            "Axum",
            Node::new("Axum", 66, vec![("Shire", 2), ("Adwa", 1), ("Asmera", 5)]),
        ),
        (
            "Dega Habur",
            Node::new("Dega Habur", 45, vec![("Jigjiga", 5), ("Kebri Dehar", 6)]),
        ),
        ("Werder", Node::new("Werder", 46, vec![("Kebri Dehar", 6)])),
        (
            "Asmera",
            Node::new("Asmera", 68, vec![("Adigrat", 9), ("Axum", 5)]),
        ),
        ("Juba", Node::new("Juba", 50, vec![("Bench Maji", 22)])),
        (
            "Kartum",
            Node::new("Kartum", 81, vec![("Metema", 19), ("Humera", 21)]),
        ),
        (
            "Mogadishu",
            Node::new("Mogadishu", 40, vec![("Gode", 22), ("Moyale", 40)]),
        ),
        ("Nairobi", Node::new("Nairobi", 22, vec![("Moyale", 22)])),
    ]);

    let initial_state = "Addis Ababa";
    let goal_state = "Moyale";

    let addis_ababa_to_moyale_searcher =
        InformedSearch::new(state_space_graph, initial_state, goal_state);

    let path_with_a_start = addis_ababa_to_moyale_searcher.try_search(Strategies::ASTAR);
    pretty_print_path(path_with_a_start, Strategies::ASTAR);

    let path_with_a_start = addis_ababa_to_moyale_searcher.try_search(Strategies::GREEDY);
    pretty_print_path(path_with_a_start, Strategies::GREEDY);
}
