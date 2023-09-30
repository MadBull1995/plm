from generated.clients.python import RegistryService_v1,registry_v1
client = RegistryService_v1(None, {
    'host':'localhost',
    'port':7575
})

md_request = registry_v1.MetadataRequest(
    library="@cncf/udpa"
)
response = client.Metadata(md_request)
print(response)
