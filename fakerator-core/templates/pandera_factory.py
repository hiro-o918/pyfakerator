class {{ record_class_name }}(TypedDict):
{%- for field in fields %}
    {{ field.name }}: {{ field.get_type_annotation() }}
{%- endfor %}


def {{ record_factory_name }}(
    *,
{%- for field in fields %}
    {{ field.name }}: {{ field.get_type_annotation() }} | f.Unset = f.Unset,
{%- endfor %}
) -> {{ record_class_name }}:
    return {
{%- for field in fields %}
        "{{ field.name }}": f.Unset.unwrap_or_else(lambda: f.{{ field.get_faker_method() }}),
{%- endfor %}
    }

