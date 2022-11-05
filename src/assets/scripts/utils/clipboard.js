fallbackCopyTextToClipboard = (text) => {
    let textArea = document.createElement("textarea");
    textArea.value = text;
    
    textArea.style.top = "0";
    textArea.style.left = "0";
    textArea.style.position = "fixed";
  
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
  
    try {
      document.execCommand('copy');
    } catch (err) {}
  
    document.body.removeChild(textArea);
  }

  copyTextToClipboard = async (text) => {
    if (!navigator.clipboard) {
      fallbackCopyTextToClipboard(text);
    }

    navigator.clipboard.writeText(text);
  }