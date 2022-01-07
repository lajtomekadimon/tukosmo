
function el_classes_selectFile() {
    const classes_selectFile =
        document.querySelectorAll('.select-file');

    classes_selectFile.forEach(
        el => el.addEventListener(
            'click',
            event => {
                const file_id = event.currentTarget.dataset.id;
                const file_name = event.currentTarget.dataset.name;
                const file_route =
                    event.currentTarget.dataset.route;

                const id_fileInput = getId("file-input");
                id_fileInput.value = file_id;

                const id_fileName = getId("file-name");
                id_fileName.style.display = "none";

                const id_fileImg = getId("file-img");
                id_fileImg.src = file_route;
                id_fileImg.alt = file_name;

                const id_fileImgcard = getId("file-imgcard");
                id_fileImgcard.style.display = "block";

                const id_fileSelectorCancel = getId("file-selector-cancel");
                id_fileSelectorCancel.classList.remove('is-hidden');

                const id_fileSelectorPanel = getId("file-selector-panel");
                id_fileSelectorPanel.classList.remove('is-active');
            }
        )
    );
}

//el_classes_selectFile();
