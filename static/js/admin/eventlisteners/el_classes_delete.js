
function el_classes_delete() {
    const classes_delete = document.querySelectorAll('.delete');

    classes_delete.forEach(
        el => el.addEventListener(
            'click',
            event => {
                if (event.currentTarget.id === "languages-close") {
                    // Do nothing
                } else {
                    const parent_node = event.currentTarget.parentNode;
                    parent_node.style.opacity = '0';
                }
            }
        )
    );
}

el_classes_delete();
