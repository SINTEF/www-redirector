# www-redirector

This is a tiny HTTP server that redirects all requests permanently to a another server.

## Configuration

Use the `REDIRECT_URL` environment variable to set the URL to redirect to. The path and query string of the request will be appended to the URL.

## Why

Sometimes it is simpler to have a tiny HTTP server to handle redirects than to configure a web server to do the redirects.
You could do the same thing using any web server, such as Caddy or Nginx.

This is just a tiny server written mostly for fun.

## License

The source code is released under the WTFPL license. See the LICENSE file for more information.