export default url => {
    let params = {}

    const paramStringSplit = url.split("?")
    if (paramStringSplit.length !== 2) {
        return params
    }
    const paramString = paramStringSplit[1]

    const paramStrings = paramString.split("&")
    if (!paramString || paramString.length === 0) {
        return params
    }

    paramStrings.forEach(p => {
        const pSplit = p.split("=")
        if (pSplit.length === 2) {
            params[pSplit[0]] = pSplit[1]
        }
    });
    return params
}