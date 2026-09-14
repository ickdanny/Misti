

fn main() {
    println!("cargo:rerun-if-changed=c/*");

    cc::Build::new()
        .include("c/MokyoMidi")
        .include("c/Constructure")
        .include("c/PGUtil")
        .include("c/Trifecta")
        .include("c/ZMath")
        .files([
            "c/MokyoMidi/MokyoMidi_Constants.c",
            "c/MokyoMidi/MokyoMidi_MidiHub.c",
            "c/MokyoMidi/MokyoMidi_MidiOut.c",
            "c/MokyoMidi/MokyoMidi_MidiSequence.c",
            "c/MokyoMidi/MokyoMidi_MidiSequencer.c",
            "c/PGUtil/PGUtil_Alloc.c",
            "c/PGUtil/PGUtil_Error.c",
            "c/PGUtil/PGUtil.c",
            "c/Trifecta/Trifecta_Time.c",
            "c/Trifecta/Trifecta_Concurrency.c",
            "c/Constructure/Constructure_ArrayList.c",
            "c/ZMath/ZMath_Bitwise.c",
            "c/ZMath/ZMath_Numeric.c",
        ])
        .flag("/std:c11")
        .flag("/experimental:c11atomics")
        .define("WIN32", None)
        // .define("VERBOSE", None)
        .compile("mokyo_midi");
    
    println!("cargo:rustc-link-lib=winmm");
}