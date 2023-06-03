<template>
  <v-card rounded="lg">
    <v-card-title class="text-h6">{{ props.item.filename }}</v-card-title>
    <v-card-subtitle>{{ date }}</v-card-subtitle>
    <v-card-text>
      <v-img
        class="image"
        v-bind="props"
        :aspect-ratio="4 / 3"
        cover
        rounded="lg"
        :src="imageSrc"
      >
        <template v-slot:error v-slot:placeholder>
          <div
            class="d-flex align-center justify-center fill-height border image"
          >
            <v-icon
              size="128px"
              :icon="
                props.item.type === 'file'
                  ? 'mdi-file-outline'
                  : 'mdi-file-document-outline'
              "
            ></v-icon>
          </div>
        </template>
      </v-img>
    </v-card-text>
    <v-card-actions class="justify-end">
      <v-btn :to="route" text>Go to</v-btn>
      <v-btn text @click="downloadButton">Download</v-btn>
      <v-btn v-if="itsMe" text @click="deleteButton">Delete</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup>
import { downloadPaste } from "~/api";

const props = defineProps({
  item: {
    required: true,
  },
  itsMe: {
    required: false,
    default: false,
  },
});

const emit = defineEmits(["delete"]);

const paste = ref(null);
const imageSrc = ref(
  await loadImage(props.item.link, props.item.filename, props.item.mime)
);
const link = ref(props.item.link);
const route = ref(`${props.item.type === "file" ? "/f" : "/p"}/${link.value}`);
const date = ref(dateParse(props.item.createdAt).toLocaleString("en-GB", {
  hour: "2-digit",
  minute: "2-digit",
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
}));
date.value = date.value.replace(",", " at");

async function loadImage(link, filename, mime) {
  const { paste: pasteBinary, error: errorBinary } = await downloadPaste(
    link,
    filename
  );

  if (!errorBinary) {
    const image = await blobToImage(pasteBinary.value, mime);
    paste.value = pasteBinary.value;
    return image;
  } else {
    return "http://localhost/bad/request.png";
  }
}

function downloadButton() {
  let url = URL.createObjectURL(paste.value);
  let link = document.createElement("a");
  link.href = url;
  link.download = props.item.filename;
  link.click();
}

function deleteButton() {
  emit("delete", props.item.link);
}

function dateParse(s) {
  let b = s.split(/\D/);
  --b[1];
  b[6] = b[6].substr(0, 3);
  return new Date(Date.UTC(...b));
}
</script>

<style scoped>
.image {
  border-radius: 8px;
}
</style>
