use std::{
    convert::TryFrom,
    error::Error,
    fs
};

// use clap::{arg, Command};
use midir::{MidiOutput, MidiOutputConnection};
use nodi::{
	midly::{Format, Smf},
	timers::Ticker,
	Player, Sheet,
};

struct Args {
	file: String,
	device_no: usize,
	list: bool,
}

impl Args {
    fn from_args() -> Self {
        Self{
            file: String::from("test.mid"),
            device_no: 0,
            list: true
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.list {
            return list_devices();
        }

        let data = fs::read(&self.file)?;
        let Smf { header, tracks } = Smf::parse(&data)?;
        let timer = Ticker::try_from(header.timing)?;

        let con = get_connection(self.device_no)?;

        let sheet = match header.format {
            Format::SingleTrack | Format::Sequential => Sheet::sequential(&tracks),
            Format::Parallel => Sheet::parallel(&tracks),
        };

        let mut player = Player::new(timer, con);

        println!("starting playback");
        player.play(&sheet);
        Ok(())
    }
}

fn get_connection(n: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("Misti")?;

    let out_ports = midi_out.ports();
    if out_ports.is_empty() {
        // here, .into() coerces into Box<dyn Error>> I think...
        return Err("no MIDI output device detected".into());
    }
    if n >= out_ports.len() {
        return Err(format!(
            "only {} MIDI devices detected; run with --list to see them",
            out_ports.len()
        ).into());
    }

    let out_port = &out_ports[n];
    let out = midi_out.connect(out_port, "todo idk what this string does")?;
    Ok(out)
}

fn list_devices() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Misti")?;

    let out_ports = midi_out.ports();

    if out_ports.is_empty() {
        println!("No active MIDI output device detected.");
    } else {
        for (i, p) in out_ports.iter().enumerate() {
            println!(
                "#{}: {}",
                i,
                midi_out.port_name(p)
                        .as_deref()
                        .unwrap_or("<no device name>")
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    Args::from_args().run()
}