
function el_classes_fileSelectorClose() {
    const classes_fileSelectorClose =
        document.querySelectorAll('.file-selector-close');

    classes_fileSelectorClose.forEach(
        el => el.addEventListener(
            'click',
            event => {
                const id_fileSelectorPanel = getId("file-selector-panel");
                id_fileSelectorPanel.classList.remove('is-active');
            }
        )
    );
}

//el_classes_fileSelectorClose();
