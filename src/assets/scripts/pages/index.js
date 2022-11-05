copy_hash_to_clipboard = (hash) => {
    copyTextToClipboard(`${window.location.protocol}//${window.location.host}/h/${hash}`);
    
    const copypaste_confirm_elem = document.getElementById("copypaste-confirm");
    copypaste_confirm_elem.classList.remove("invisible");

    setTimeout(() => {
        copypaste_confirm_elem.classList.add("invisible");
    }, 2000);
}