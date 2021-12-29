
const classes_notification = document.querySelectorAll('.notification');

classes_notification.forEach(
    el => el.addEventListener(
        'transitionend',
        event => {
            event.currentTarget.remove();
        }
    )
);

