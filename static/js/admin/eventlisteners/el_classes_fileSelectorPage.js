
function el_classes_fileSelectorPage(function_value) {
    const classes_fileSelectorPage =
        document.querySelectorAll('.file-selector-page');

    classes_fileSelectorPage.forEach(
        el => el.addEventListener(
            'click',
            event => {
                const new_route = event.currentTarget.dataset.route;
                function_value(new_route);
            }
        )
    );
}

//el_classes_fileSelectorPage();
