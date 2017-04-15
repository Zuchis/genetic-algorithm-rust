#!/bin/bash

COMP=pdflatex #-no-file-line-error
BIB_COMP=bibtex
MAIN=sbc-template
MAIN_BIB=sb-template

${COMP}       ${MAIN}
${BIB_COMP}   ${MAIN}
${COMP}       ${MAIN}
${COMP}       ${MAIN}
