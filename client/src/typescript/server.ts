// @ts-ignore
export const server_url = SERVER_ADDRESS;

export function route(routename: string): string {
    return `${server_url}/${routename}`.replace("//", "/").replace("//", "/").replace("http:/", "http://")
}
