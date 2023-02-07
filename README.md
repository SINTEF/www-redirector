# www-redirector

This is a tiny HTTP server that redirects all requests permanently to another server.

## Configuration

Use the `REDIRECT_URL` environment variable to set the URL to redirect to. The path and query string of the request will be appended to the URL.

## Sotware Container Image

```
ghcr.io/sintef/www-redirector
```

### Helm Chart

A Helm chart is available in the [charts/www-redirector](charts/www-redirector/) directory.

## Why

Sometimes it is fun to over engineer things.

It can also be simpler to use a tiny HTTP server to handle redirects compared to configuring a web server to do the redirects correctly. Especially when you are using an Kubernetes ingress controller or some load balancers and you don't feel like spending hours to figure out how to configure them correctly.

You could do the same thing using any web server, such as Caddy or Nginx. But the 50 lines of rust code work too.

## License

The source code is released under the WTFPL license. See the LICENSE file for more information.