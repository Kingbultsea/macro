use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "菊花残，满地伤~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

pub fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}