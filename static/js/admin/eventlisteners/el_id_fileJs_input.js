
function el_id_fileJs_input() {
    const elem = document.querySelector('#file-js input[type=file]');

    if (elem !== null) {
        elem.onchange = () => {
            if (elem.files.length > 0) {
                const class_fileName =
                    document.querySelector('#file-js .file-name');
                class_fileName.textContent = elem.files[0].name;
            }
        }
    }
}

el_id_fileJs_input();
