from io import BytesIO

from openai import OpenAI
from PIL import Image
client = OpenAI(api_key='')


img=Image.open('fer.jpeg')
width, height = 256, 256
image = img.resize((width, height))

image=image.convert('RGBA')
byte_stream = BytesIO()
image.save(byte_stream, format='PNG')
byte_array = byte_stream.getvalue()

response = client.images.edit(
    image=byte_array,
    n=2,
    size="256x256",
    prompt='draw big eyes'
)

image_url = response.data[0].url

print(image_url)
