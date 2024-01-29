# notes on running:  https://docs.streamlit.io/knowledge-base/deploy/remote-start

import streamlit as st
from oalib.solutions import create_code

st.title("Code Generator")
st.write("Convert a comment into code in any language")

language = st.text_input("Language", "python")
text = st.text_area("Comment", "Calculate the mean distance between an array of points")

if st.button("Generate Code"):
    with st.spinner(text="In progress"):
        code = create_code(text, language)
        st.code(code)
