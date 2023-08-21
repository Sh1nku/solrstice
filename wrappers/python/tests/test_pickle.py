import pickle

from solrstice.group import GroupingComponent
from solrstice.queries import CommitType, DeleteQuery, SelectQuery, UpdateQuery


def test_pickle_works_select_query_builder():
    builder = SelectQuery(
        fq=["test", "test"],
        grouping=GroupingComponent(fields=["test"], main=True, facet=False),
    )
    string = pickle.dumps(builder)
    builder_copy: SelectQuery = pickle.loads(string)
    assert pickle.dumps(builder_copy) == string


def test_pickle_works_update_query_builder():
    builder = UpdateQuery(handler="test", commit_type=CommitType.Soft)
    string = pickle.dumps(builder)
    builder_copy: UpdateQuery = pickle.loads(string)
    assert pickle.dumps(builder_copy) == string


def test_pickle_works_delete_query_builder():
    builder = DeleteQuery()
    string = pickle.dumps(builder)
    builder_copy: DeleteQuery = pickle.loads(string)
    assert pickle.dumps(builder_copy) == string
