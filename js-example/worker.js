onmessage = async function (e) {

    for await (const [key, value] of e.data.entries()) {
        console.log({ key, value });

        // do something interesting here
    }

}