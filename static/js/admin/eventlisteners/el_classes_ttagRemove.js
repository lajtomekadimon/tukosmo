
function el_classes_ttagRemove() {
    const classes_ttagRemove = document.querySelectorAll('.ttag-remove');

    classes_ttagRemove.forEach(
        el => el.addEventListener(
            'click',
            event => {
                const tag_id = el.dataset.id;
                const tagInput = getId("tag-input-" + tag_id);
                tagInput.remove();
                const tagLabel = getId("tag-label-" + tag_id);
                tagLabel.remove();
            }
        )
    );
}

el_classes_ttagRemove();
