export async function createImage(src: string): Promise<HTMLImageElement> {
    const image = new Image();
    image.src = src;
    return new Promise((resolve, reject) => {
        image.onload = () => {
            return resolve(image);
        };

        image.onerror = e => {
            return reject(e);
        };
    });
}
