
const id_fileJs_input = document.querySelector('#file-js input[type=file]');

if (id_fileJs_input !== null) {
    id_fileJs_input.onchange = () => {
        if (id_fileJs_input.files.length > 0) {
            const class_fileName =
                document.querySelector('#file-js .file-name');
            class_fileName.textContent = id_fileJs_input.files[0].name;
        }
    }
}

