<template>
<div class="library">
	<div class="books">
		<h1>Library</h1>
		<!-- let's do a grid of book shaped divs, each has a large capital letter of the title[0] and then the title and author at the bottom -->
		<div class="book-grid">
		<!-- a head with names of the columns -->
			<div class="book-grid-head">
				<div class="book-grid-head-column">Title</div>
				<div class="book-grid-head-column">Author</div>
			</div>
			<div v-for="book in library" class="book" @click="showBookDetail(book)">
				<div class="cell book-title">{{ book.title }}</div>
				<div class="cell book-author">{{ book.author }}</div>
			</div>
		</div>
	</div>
	<div class="detail-modal">
		<div v-if="selectedBook" class="modal-background">
			<div class="modal-content">
			<h2>{{ selectedBook.title }}</h2>
			<p>{{ selectedBook.author }}</p>
			<!-- chapters -->
			<div class="chapters">
				<div v-for="(chapter, index) in selectedBook.chapters" :key="index" :class="((playingChapter === chapter.file) ? 'chapter-playing chapter' : 'chapter')" @click="playChapter(chapter.file, chapter.title)">
					<span>{{ index + 1 }}.</span>
					<span>{{ chapter.title }}</span>
					<span>{{ secondsToPrettyTime(chapter.length) }}</span>
				</div>
			</div>
			<button @click="closeBookDetail">Close</button>
			</div>
		</div>
	</div>
</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { homeDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

// the library is a prop
export default {
	props: ["library"],
	data() {
		return {
			selectedBook: null,
			playingChapter: null,
		};
	},
	methods: {
		showBookDetail(book) {
			this.selectedBook = book;
		},
		closeBookDetail() {
			this.selectedBook = null;
		},
		playChapter(file, title) {
			this.playingChapter = file;
			this.$emit("play-chapter", file, title);
		},
		secondsToPrettyTime(seconds) {
			const hours = Math.floor(seconds / 3600);
			const minutes = Math.floor(seconds / 60) % 60;
			const secondsLeft = seconds % 60;
			if (hours > 0) {
				return `${hours}:${minutes}:${secondsLeft.toFixed(0)}`;
			} else {
				return `${minutes}:${secondsLeft.toFixed(0)}`;
			}
			}
	},
};
</script>