from uwuifier import uwuify

import time
from faker import Faker

print("Benchmarking:")
print("Generating texts...")

faker = Faker()

# Generate a list of fake texts
num_texts = 10000
fake_texts = [faker.text(1000) for _ in range(num_texts)]
characters = sum([len(text) for text in fake_texts])

time.sleep(3)

print("Benchmarking...")
# Measure the execution time of uwuify function
start_time = time.time()
for i, text in enumerate(fake_texts):
    uwuified_text = uwuify(text)
end_time = time.time()

execution_time = end_time - start_time
print(f"Execution time for {num_texts} fake texts ({characters} characters): {execution_time:.4f} seconds")
print(f"Average per text: {execution_time/num_texts:.8f} seconds")
print(f"Average per character: {execution_time/characters:.12f} seconds")
print("\n===\n")