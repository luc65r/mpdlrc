use std::{
    fs::File,
    io::Read,
    path::Path,
    thread::sleep,
    time::Duration,
};
use mpd::Client;
use lrc::Lyrics;

fn main() {
    let music_dir = Path::new("/home/lucas/Musique");

    let mut conn = Client::connect("127.0.0.1:6600")
        .expect("Can't connect to mpd");

    let mut mus_file = String::new();
    let mut contents = String::new();
    let mut lyrics = Vec::new();

    loop {
        let new_mus_file = { 
            let mut m = conn.currentsong().expect("Can't communicate with mpd");
            while m == None {
                sleep(Duration::from_millis(100));
                m = conn.currentsong().expect("Can't communicate with mpd");
            }
            m.unwrap().file
        };

        if mus_file != new_mus_file {
            mus_file = new_mus_file;
            let mut file = File::open(music_dir.join(&mus_file).with_extension("lrc")).unwrap();

            contents.clear();
            file.read_to_string(&mut contents).unwrap();
            lyrics = Lyrics::from_str(&contents)
                .unwrap()
                .get_timed_lines()
                .to_owned()
                .iter().rev()
                .map(|(t, s)| (t.get_timestamp(), s.to_string()))
                .collect::<Vec<(i64, String)>>();

            println!("\n\n");
        }

        let elapsed = conn.status()
            .expect("Can't communicate with mpd")
            .elapsed
            .expect("No song is playing")
            .num_milliseconds();

        while let Some(l) = lyrics.last() {
            if l.0 <= elapsed {
                println!("{}", lyrics.pop().unwrap().1);
            } else {
                break;
            }
        }

        sleep(Duration::from_millis(100));
    }
}
