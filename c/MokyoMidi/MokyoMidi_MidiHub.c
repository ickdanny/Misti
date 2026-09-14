#include "MokyoMidi_MidiHub.h"

#include "PGUtil_Alloc.h"

/*
 * Constructs a new MidiHub and returns it by value.
 * Should not be called more than once
 */
MidiHub midiHubMake(
    bool muted,
    int32_t midi_out_index,
    void (*shortMsgCallback)(uint32_t)
){
    MidiHub toRet = {0};
    toRet.midiOutPtr
        = pgAlloc(1, sizeof(*(toRet.midiOutPtr)));
    *(toRet.midiOutPtr) = mmMidiOutMake(
        midi_out_index,
        shortMsgCallback
    );
    toRet.midiSequencer = midiSequencerMake(
        toRet.midiOutPtr
    );
    toRet.muted = muted;
    return toRet;
}

/* 
 * Starts playing the given midi sequence. Replaces
 * the previously playing sequence
 */
void midiHubStart(
    MidiHub *midiHubPtr, 
    MidiSequence *midiSequencePtr
){
    if(!(midiHubPtr->muted)){
        midiSequencerStart(
            &(midiHubPtr->midiSequencer),
            midiSequencePtr
        );
    }
}

/* Stops playing the current midi sequence */
void midiHubStop(MidiHub *midiHubPtr){
    midiSequencerStop(&(midiHubPtr->midiSequencer));
}

/* 
 * If midi is currently muted, unmutes it. Otherwise,
 * mutes it
 */
void midiHubToggleMute(MidiHub *midiHubPtr){
    /* if we are now muted */
	if((midiHubPtr->muted = !(midiHubPtr->muted))){
		midiHubStop(midiHubPtr);
	}
}

/*
 * Returns true if midi is currently muted, false
 * otherwise
 */
bool midiHubIsMuted(const MidiHub *midiHubPtr){
    return midiHubPtr->muted;
}

/*
 * Closes the given MidiHub and frees all associated
 * resources. Using a MidiHub after it has been closed
 * is undefined behavior
 */
void midiHubFree(MidiHub *midiHubPtr){
    midiSequencerFree(&(midiHubPtr->midiSequencer));
    mmMidiOutFree(midiHubPtr->midiOutPtr);
    pgFree(midiHubPtr->midiOutPtr);
}

/* ================================== *
 * ADDED FUNCTIONS TO MAKE FFI EASIER *
 * ================================== */

MidiHub *midiHubAlloc(
    bool muted,
    int32_t midi_out_index,
    void (*shortMsgCallback)(uint32_t)
) {
    MidiHub *midiHubPtr = pgAlloc(
        1, 
        sizeof(*midiHubPtr)
    );
    if(midiHubPtr){
        *midiHubPtr = midiHubMake(muted, midi_out_index, shortMsgCallback);
    }
    return midiHubPtr;
}
void midiHubFreeAlloc(MidiHub *midiHubPtr) {
    if(midiHubPtr){
        midiHubFree(midiHubPtr);
        pgFree(midiHubPtr);
    }
}