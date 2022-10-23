/*
Would probably be of interest to be able to set up a sequencer timeline. 
Maybe using GNU Rocket?
But _also_ need timing stuff. LFOs should be able to be synchronized by bpm.

BPM detection? How to go about it?
Metronome? 
    * vec4(beatChangedLastFrame, currentBeat, beatInBar, bar)
    * vec4(ticks?, currentBeat, beatInBar, bar) having the BPM in the shaders won't matter, it's of interest when it changes. might be more interesting to let that trigger a sequencer which takes care of the automation. 

I think automation/animation is important. Will probably come back to this to add more.
    */

