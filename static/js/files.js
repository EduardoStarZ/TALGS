document.getElementById('file').addEventListener('change', function() {

		var reader = new FileReader();
		reader.onload = function() {

				var arrayBuffer = this.result,
						array = new Uint8Array(arrayBuffer),
						binaryString = String.fromCharCode.apply(null, array);

				let hexString = Array.from(array, function(byte) {
						return ('0' + (byte & 0xFF).toString(16)).slice(-2);
				}).join('');

				document.getElementById("bytes").value = hexString;
		}
		reader.readAsArrayBuffer(this.files[0]);

		const filename = this.files[0].name;

		console.log(filename);

		document.getElementById("filename").value = filename;
}, false);
