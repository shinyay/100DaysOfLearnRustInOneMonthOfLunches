use std::env;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: comforting-words [options] [name]");
        println!("Prints a comforting message along with an optional name.");
        println!("Options:");
        println!("  -h, --help     Display this help message");
        println!("  -j, --japanese Display the message in Japanese");
        println!("If a name is provided, the message will be addressed to the name.");
        return;
    }

    let is_japanese = args.contains(&"-j".to_string()) || args.contains(&"--japanese".to_string());

    let comforting_words = if is_japanese {
        [
            "頑張っています！",
            "深呼吸してリラックスしましょう。",
            "これも過ぎ去る時です。",
            "あなたは思っているよりも強いです。",
            "ポジティブなエネルギーを送ります。",
            "あなたは一人ではありません。",
            "自分を信じてください。",
            "あなたは大丈夫です！",
            "前向きに考えて、頑張りましょう。",
            "あなたは素晴らしいことができる能力を持っています。",
            "上を向いてください！",
            "進捗しています、たとえそれが感じられなくても。",
            "あなたは正しい方向に進んでいます。",
            "あなたは本当のインスピレーションです。",
            "あなたの努力は報われます。",
            "夢を信じてください。",
            "あなたは勇敢に戦ってます！",
            "あなたは過去の出来事で決まるものではありません。",
            "集中して決して諦めないでください。",
            "あなたは輝くスターです。",
        ]
    } else {
        [
            "You're doing great!",
            "Take a deep breath and relax.",
            "This too shall pass.",
            "You're stronger than you think.",
            "Sending you positive vibes.",
            "You're not alone in this.",
            "Believe in yourself.",
            "You've got this!",
            "Stay positive and keep going.",
            "You're capable of amazing things.",
            "Keep your chin up!",
            "You're making progress, even if it doesn't feel like it.",
            "You're on the right track.",
            "You're a true inspiration.",
            "Your efforts will pay off.",
            "Believe in your dreams.",
            "You're a fighter!",
            "You're not defined by your past.",
            "Stay focused and never give up.",
            "You're a shining star.",
        ]
    };

    let name = args.get(2).cloned();

    let mut rng = rand::thread_rng();
    let random_message = comforting_words.choose(&mut rng).unwrap();

    match name {
        Some(name) => println!("{}: {}", name, random_message),
        None => println!("{}", random_message),
    }
}
