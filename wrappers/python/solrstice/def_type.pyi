import abc
from enum import Enum
from typing import Optional


class QueryOperator(Enum):
    AND = 'AND'
    OR = 'OR'

class DefType(abc.ABC):
    pass

class DefTypeLucene(DefType):
    q_op: Optional[QueryOperator]
    df: Optional[str]
    sow: Optional[bool]

    def __init__(self, q_op: Optional[QueryOperator] = None, df: Optional[str] = None, sow: Optional[bool] = None):
        pass