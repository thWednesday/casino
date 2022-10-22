#![allow(non_snake_case)]

#[derive(Debug, Clone)]
pub struct Card {
    pub value: i32,
    pub ascii: Vec<String>,
}

impl Card {
    pub fn new(value: i32, ascii: Vec<String>) -> Self {
        Self { value, ascii }
    }
}

fn cardTemplate(value: i32) -> Vec<String> {
    let mut template: Vec<String> = Vec::new();
    template.push(" __________ ".to_string());

    if value > 9 {
        template.push(format!("|{}        |", value.to_string()).to_string());
    } else {
        template.push(format!("|{}         |", value.to_string()).to_string());
    }

    let mut lines: Vec<Vec<String>> = Vec::new();

    for _i in 1..=6 {
        let mut temp = Vec::new();

        for _i in 1..=10 {
            temp.push(" ".to_string());
        }

        lines.push(temp);
    }

    for line in lines {
        template.push(format!("|{}|", line.join("")).to_string());
    }

    if value > 9 {
        template.push(format!("|        {}|", value.to_string()).to_string());
    } else {
        template.push(format!("|         {}|", value.to_string()).to_string());
    }

    template.push(" ~~~~~~~~~~ ".to_string());

    return template;
}

// fn cardTemplate(value: i32) -> Vec<String> {
//     let mut template: Vec<String> = Vec::new();
//     template.push(" __________ ".to_string());

//     if value > 9 {
//         template.push(format!("|{}        |", value.to_string()).to_string());
//     } else {
//         template.push(format!("|{}         |", value.to_string()).to_string());
//     }

//     template.push("|+         |".to_string());
//     template.push("|    +     |".to_string());
//     template.push("|          |".to_string());

//     template.push("|    +     |".to_string());
//     template.push("|         +|".to_string());

//     if value > 9 {
//         template.push(format!("|        {}|", value.to_string()).to_string());
//     } else {
//         template.push(format!("|         {}|", value.to_string()).to_string());
//     }

//     template.push(" ~~~~~~~~~~ ".to_string());

//     return template;
// }

pub fn combineCards(cards: Vec<Card>) -> Vec<String> {
    let mut deck: Vec<String> = Vec::new();

    for length in 0..cards[cards.len() - 1].ascii.len() {
        let mut temp_string: String = "".to_string();
        for card in &cards {
            temp_string = format!(
                "{} {}",
                temp_string,
                card.ascii.get(length).unwrap().to_string()
            )
            .to_string();
        }

        deck.push(temp_string);
    }

    return deck;
}

pub fn card(value: i32) -> Card {
    return Card::new(value, cardTemplate(value));
}
