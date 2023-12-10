<script setup>
import { ref, reactive } from "vue";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/tauri";
import {Howl, Howler} from 'howler';
import { appDataDir, homeDir, join } from '@tauri-apps/api/path';
import { readBinaryFile, readDir, BaseDirectory } from "@tauri-apps/api/fs"
import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri';
import Library from './Library.vue';

let loaded = ref(false);
let loading = ref(true); // this is for the initial load of the library from rust
let showSpeed = ref(false);

let appData = reactive({
  library: reactive([]),
  current_speed: 1,
});

let importStatus = reactive({
  importing: false,
  files: [],
  name: '',
  author: '',
});

// get our appData from rust
invoke("get_app_data").then((data) => {
  appData = reactive(data);
  console.log('got app data', appData);
  loading.value = false; 
});

// object with info about current playback
const stats = reactive({
  currentTrack: null,
  currentTrackDuration: null,
  currentTrackProgress: 0,
  currentTime: 0,
  playing: false,
});

// update the stats
setInterval(() => {
  if (stats.currentTrack && audio && audio.playing()) {
    stats.currentTrackProgress = audio.seek() / audio.duration() * 100;
    stats.currentTime = audio.seek();
    stats.currentTrackDuration = audio.duration();
    console.log('updating stats', stats);
  }
}, 1000);

// main audio object
let audio = ref(null);

async function addNew() {
  const selected = await open({
    directory: true,
    defaultPath: await homeDir(),
  });

   if (Array.isArray(selected)) {
      // user selected multiple directories
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      // user selected a single directory
      if (selected !== null) {
        const path = convertFileSrc(selected);
        console.log('selected', selected);
        const entries = await readDir(selected);
        importStatus.files = entries;
        importStatus.name = selected;
        importStatus.path = selected;
        importBook();
      }
    }
}

async function handleFileDrop (event) {
  console.log(event);
  // first make sure this is a folder and not a file
  try {
    const entries = await readDir(event.payload[0]);
    importStatus.files = entries;
    importStatus.name = event.payload[0];
    importStatus.path = event.payload[0];
    importBook();
  } catch (error) {
    console.log('Not a directory');
  }
}

listen('tauri://file-drop', event => {
  handleFileDrop(event)
})

async function importBook() {
  console.log('importing book', importStatus);
  importStatus.importing = true;
}

async function importBookConfirm() {
  console.log('importing book', importStatus);
  if (importStatus.files.length === 0) {
    return;
  }

  // we want to properly escape the path
  const sending = {
    name: importStatus.name,
    author: importStatus.author,
    path: importStatus.path,
  }
  // send the files to rust
  invoke("import_book", { book: sending }).then((data) => {
    console.log('imported book', data);
    importStatus.files = [];
    if (data === 'OK') {
      // we need to update our appData
      invoke("get_app_data").then((data) => {
        loading.value = false;
        appData = data;
        console.log('got app data', appData);
        loading.value = false;
      });
    }
  });
  importStatus.importing = false;
  importStatus.name = '';
  importStatus.author = '';
  importStatus.path = '';
}

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
            try {
              audio.unload();
            } catch (_) {}
            audio = new Howl({
              src: [base64data],
              html5: true, // this is so we can change playback rate without the pitch increasing
              ext: [stats.currentTrack.split('.').pop()],
            });
            audio.play();
            audio.rate(appData.current_speed);
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

function togglePlay(){
  stats.playing = !stats.playing;
  return audio.playing() ? audio.pause() : audio.play();
};

// catch spacebar to play/pause
document.addEventListener('keydown', function(event) {
  if(event.keyCode === 32) {
    event.preventDefault();
    togglePlay();
  }
});

function rewind() {
  audio.seek(audio.seek() - 10);
}

function skip() {
  audio.seek(audio.seek() + 10);
}

function changecurrent_speed() {
  audio.rate(appData.current_speed);
}

function seekAudioFile() {
  // const seek = audio.duration() / 100 * parseInt(stats.currentTrackProgress);
  // console.log('seeking to', seek);
  audio.pause();
  // they're messing with me in the docs, this expects a percentage...
  //audio.seek(parseInt(stats.currentTrackProgress));
  audio.seek(audio.duration() / 100 * parseInt(stats.currentTrackProgress));
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

    console.log(selected);

    if (selected !== null) {
        const path = convertFileSrc(selected);
        stats.currentTrack = path;
        stats.playing = true;
        playAudioFile();
      }
}

function playChapter(chapter, title) {
  stats.playing = false;
  console.log('playing chapter', chapter);
  const path = convertFileSrc(chapter);
  stats.currentTrack = path;
  stats.currentTitle = title;
  stats.playing = true;
  playAudioFile();
}

function secondsToPrettyTime(seconds) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor(seconds / 60) % 60;
  const secondsLeft = seconds % 60;
  if (hours > 0) {
    return `${hours}:${minutes}:${secondsLeft.toFixed(0)}`;
  } else {
    return `${minutes}:${secondsLeft.toFixed(0)}`;
  }
}

console.log('loaded player', appData);
loaded.value = true;

</script>

<template>
  <div v-if="loaded">
    <div ref="importDialog" v-if="importStatus.importing" class="importDialog">
      <h1>Import</h1>
      <!-- set custom name -->
      <div>
        <label for="name">Name</label>
        <input type="text" id="name" v-model="importStatus.name" />
        <label for="author">Author</label>
        <input type="text" id="author" v-model="importStatus.author" />
      </div>
      <p>The following files will be imported:</p>
      <ul>
        <li v-for="file in importStatus.files">
          {{ file.name }}
          <ul v-if="file.children">
            <li v-for="child in file.children">
              {{ child.name }}
            </li>
          </ul>
        </li>
      </ul>
      <button @click="importBookConfirm()">OK</button>
    </div>

    <Library v-if="!loading.value && appData && appData.library" :library="appData.library" @playChapter="playChapter" />

    <!-- sticky bar at the bottom with play/pause, rewind, skip, and seek -->
    <div class="sticky-bar">
      <div class="one">
        <p v-if="!stats.currentTitle">Welcome to Marble</p>
        <p v-else-if="stats.currentTitle.length <= 43">Playing: {{ stats.currentTitle }}</p>
        <p v-else>Playing: {{ stats.currentTitle.slice(0, 15) }} ... {{ stats.currentTitle.slice(-25) }}</p>
      </div>
      <div class="wide">
        <div class="controls">
          <img src="/replay_10_FILL1_wght400_GRAD0_opsz24.svg" class="icon" @click="rewind()"/>
          <img :src="(stats.playing) ? '/pause_circle_FILL1_wght400_GRAD0_opsz24.svg' : '/play_circle_FILL1_wght400_GRAD0_opsz24.svg'" @click="togglePlay()" class="icon large-icon" />
          <img src="/forward_10_FILL1_wght400_GRAD0_opsz24.svg" class="icon" @click="skip()" />
        </div>
        <div class="progress">
          {{ secondsToPrettyTime(stats.currentTime) }}
          <input type="range" min="0" max="100" v-model="stats.currentTrackProgress" @change="seekAudioFile()" />
          {{ secondsToPrettyTime(stats.currentTrackDuration) }}
        </div>
      </div>
      <!-- first 10 letters of current track ... last 10 letters of current track -->
      <div class="two">
        <div>
          <img src="/folder_FILL0_wght400_GRAD0_opsz24.svg" class="icon" @click="addNew()"/>
          <p>Import</p>
          <!--Import: <button @click="addNew()">Select new directory</button>-->
        </div>
        <div>
          <!-- playback rate number input. 0 to 10, increase by 0.1 -->
          <div class="row icon-wrapper" @click="showSpeed = !showSpeed">
            <img src="/speed_FILL0_wght400_GRAD0_opsz24.svg" class="icon" />
            <!-- click to show the number input -->
            <span class="current-speed" >{{ appData.current_speed }}x</span>
          </div>
          <input type="number" min="0" max="10" step="0.1" v-model="appData.current_speed" @input="changecurrent_speed" v-if="showSpeed" />
        </div>
      </div>
    </div>
</div>

</template>
