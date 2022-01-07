
function el_id_navbarLink() {
    const elem = getId("navbar-link");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_navbarDropdown = getId("navbar-dropdown");
                // TODO: Use add() or remove()
                id_navbarDropdown.classList.toggle('is-active');
            }
        );
    }
}

el_id_navbarLink();
