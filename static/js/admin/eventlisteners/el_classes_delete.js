
function el_classes_delete() {
    const classes_delete = document.querySelectorAll('.delete');

    classes_delete.forEach(
        el => el.addEventListener(
            'click',
            event => {
                event.currentTarget.parentNode.style.opacity = '0';
            }
        )
    );
}

el_classes_delete();
