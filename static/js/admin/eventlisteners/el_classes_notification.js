
function el_classes_notification() {
    const classes_notification = document.querySelectorAll('.notification');

    classes_notification.forEach(
        el => el.addEventListener(
            'transitionend',
            event => {
                event.currentTarget.remove();
            }
        )
    );
}

el_classes_notification();
