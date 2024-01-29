"""
squad_dataset = load_dataset('squad')
print(squad_dataset['train'][0])
"""

from datasets import load_dataset
import openai
import os

def submit_question(text):
    """This submits a question to the OpenAI API"""

    openai.api_key = os.getenv("OPENAI_API_KEY")

    result = openai.Completion.create(
        prompt=text,
        temperature=0,
        max_tokens=300,
        top_p=1,
        frequency_penalty=0,
        presence_penalty=0,
        model="text-davinci-002",
    )["choices"][0]["text"].strip(" \n")
    return result


def squad_dataset():
    dataset = load_dataset("squad")
    return dataset

def show_before_after(text):
    print(f"Question: {text}\n\n")
    print(f"####Answer: {submit_question(text)}\n\n")

payload = squad_dataset()['train'][0]['context']
show_before_after(payload)
