<template>
    <v-dialog v-model="dialog" activator="parent" width="512">
        <v-card>
            <v-card-title>Save Text</v-card-title>
            <v-card-text>
                <v-text-field
                    v-model="filename"
                    label="Filename"
                    outlined
                    dense
                ></v-text-field>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn text @click="dialog = false">Cancel</v-btn>
                <v-btn text @click="saveFile">Save</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup>
    const dialog = ref(false);
</script>