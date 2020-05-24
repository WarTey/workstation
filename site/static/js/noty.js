function noty(message, success) {
    new Noty({
        theme: 'bootstrap-v4',
        type: success ? 'success' : 'error',
        timeout: 3500,
        text: message,
    }).show();
}
