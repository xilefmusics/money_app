import saveFile from './saveFile'

export default transactions => {
    const blob = new Blob([JSON.stringify(transactions)], {type: "octet/stream"});
	saveFile(blob, "transactions.json");
}