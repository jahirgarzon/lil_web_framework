const http = require("http");
const asyncGet = (url, options) => new Promise((resolve, reject) => {
	http.get(url, options, (response) => {
		let body = '';
		response.on('data', (chunk) => { body += chunk; });
		response.on('end', () => resolve(body));
	}).on('error', reject);
});
const reqs = Array.from({length:Number(process.env.REQS)}, async (_a,b)=>{
    await asyncGet("http://localhost:7878",{});
    console.log(b);
    return b;
});


Promise.all(reqs)
.then((result)=>console.log(result))
