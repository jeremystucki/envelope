const js = import('./node_modules/envelope/envelope_wasm.js');

js.then(js => {
    document.getElementById('button').addEventListener('click', () => {
        let sender = document.getElementById('sender').value
        let recipient = document.getElementById('recipient').value

        let output = js.generate_envelope(sender, recipient);

        let a = document.createElement('a');
        a.href = URL.createObjectURL(new Blob([output], { type: 'application/pdf' }));
        a.download = 'envelope.pdf';

        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    });
});
