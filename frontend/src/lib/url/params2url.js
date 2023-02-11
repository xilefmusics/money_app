export default (url, params) => {
    const paramStrings = Object.entries(params)
        .filter(([_, value]) => value !== undefined)
        .map(([key,value]) => `${key}=${value}`)
    if (!paramStrings) {
        return url
    }
    const paramString = paramStrings.join("&")
    return `${url}?${paramString}`
}