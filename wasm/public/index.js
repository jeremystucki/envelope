const js = import('./node_modules/envelope/envelope_wasm.js');

js.then(js => {
    document.getElementById('button').addEventListener('click', () => {
        let output = js.test();
        let a = document.createElement('a');
        a.href = URL.createObjectURL(new Blob([output], { type: 'application/pdf' }));
        a.download = 'envelope.pdf';

        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    });
});
