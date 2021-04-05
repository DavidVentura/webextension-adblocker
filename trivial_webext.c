#include <gtk/gtk.h>
#include <webkit2/webkit-web-extension.h>

static gboolean
web_page_send_request (WebKitWebPage     *web_page,
                       WebKitURIRequest  *request,
                       WebKitURIResponse *redirected_response,
                       gpointer           user_data)
{
    const char *request_uri;
    const char *page_uri;

    request_uri = webkit_uri_request_get_uri (request);
    page_uri = webkit_web_page_get_uri (web_page);

    g_print("Req uri: %s\n", request_uri);
    g_print("page uri: %s\n", page_uri);
    if (strstr(request_uri, "css") != NULL) {
        g_print("Blocked!! %s\n", request_uri);
	return true;
    }
    return false;
}

static void
web_page_created_callback (WebKitWebExtension *extension,
                           WebKitWebPage      *web_page,
                           gpointer            user_data)
{
    g_print ("From extension: page %lu created for %s\n", 
             webkit_web_page_get_id (web_page),
             webkit_web_page_get_uri (web_page));
    g_signal_connect_object (web_page, "send-request",
                             G_CALLBACK (web_page_send_request),
                             NULL, 0);
}

G_MODULE_EXPORT void
webkit_web_extension_initialize (WebKitWebExtension *extension)
{
    g_signal_connect (extension, "page-created", 
                      G_CALLBACK (web_page_created_callback), 
                      NULL);
}
