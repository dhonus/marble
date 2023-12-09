<script setup>
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {Howl, Howler} from 'howler';
import { appDataDir, join } from '@tauri-apps/api/path';
import { readBinaryFile } from "@tauri-apps/api/fs"
import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri';

// object with info about current playback
const stats = reactive({
  currentTrack: null,
  currentTrackDuration: null,
  currentTrackProgress: null,
  playbackRate: 1,
});

// update the stats
setInterval(() => {
  if (stats.currentTrack && audio) {
    stats.currentTrackProgress = audio.seek();
    stats.currentTrackDuration = audio.duration();
  }
}, 1000);

const greetMsg = ref("");
const name = ref("");

let audio = ref(null);

const selectedImagePath = ref("");

async function playAudioFile() {
  const { currentTrack } = stats
  
  if (!currentTrack) {
    return
  }

  fetch(currentTrack)
    .then((res) => {
      res
        .blob()
        .then((blob) => {
          // funny workaround until we get this merged:
          // https://bugs.webkit.org/show_bug.cgi?id=146351
          // consumes more memory than necessary, so that sucks
          // but it works-ish for now and we can use Tauri
          const reader = new FileReader();
          reader.readAsDataURL(blob);
          reader.onloadend = function() {
            const base64data = reader.result;
            audio = new Howl({
              src: [base64data],
              html5: true, // this is so we can change playback rate without the pitch increasing
              ext: [stats.currentTrack.split('.').pop()],
            });
            audio.play();
            audio.rate(stats.playbackRate);
            console.log('playing audio', audio);
          }
        })
        .catch((err) => {
          console.error(err)
        })
    })
    .catch((err) => {
      console.error(err)
    })
};

function pauseAudioFile() {
  audio.pause();
}

function togglePlay(){
  return audio.playing() ? audio.pause() : audio.play();
};

function changePlaybackRate() {
  audio.rate(stats.playbackRate);
}

function seekAudioFile() {
  audio.pause();
  const seek = audio.duration() * (stats.currentTrackProgress / 100);
  console.log('seeking to', seek);
  audio.seek(seek);
  audio.play();
}

async function selectAudioFile() {
    stats.playing = false;
  
    const selected = await open({
      multiple: false,
      filters: [{
        name: "Audio",
        extensions: ["mp3", "wav", "ogg"]
      }]
    });
  
    if (selected !== null) {
      const path = convertFileSrc(selected);
      stats.currentTrack = path;
      stats.playing = true;
      playAudioFile();
    }
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  selectAudioFile();
  // selectImage();
}
</script>

<template>

  <!-- play pause buttons etc -->
  <div class="row">
    <button @click="playAudioFile()">Play</button>
    <button @click="pauseAudioFile()">Pause</button>
    <button @click="stopAudioFile()">Stop</button>
    <button @click="togglePlay()">Toggle</button>
  </div>

  <!-- progress bar where we can move around -->
  <div class="row">
    <input type="range" min="0" max="100" v-model="stats.currentTrackProgress" @change="seekAudioFile()" />
  </div>

  <p>Selected image path: {{ selectedImagePath }}</p>
  <p>Percentage of progress: {{ stats.currentTrackProgress }}</p>

  <img :src="selectedImagePath" className="h-200"/>

  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <!-- playback rate number input. 0 to 10, increase by 0.1 -->
  <div class="row">
    Playback speed
    <input type="number" min="0" max="10" step="0.1" v-model="stats.playbackRate" @input="changePlaybackRate" />
  </div>

  <!-- time remaining, but actual time accoding to current playback rate. Limit to seconds-->
  <div class="row">
    Time remaining: {{ ((stats.currentTrackDuration - stats.currentTrackProgress) / stats.playbackRate).toFixed(0) }}
  </div>

  <p>{{ greetMsg }}</p>
</template>
